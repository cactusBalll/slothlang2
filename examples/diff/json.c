#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef struct { char *d; size_t len, cap; } Buf;
static void binit(Buf *b) { b->d = NULL; b->len = 0; b->cap = 0; }
static void bputc(Buf *b, char c) {
    if (b->len + 1 >= b->cap) { b->cap = b->cap ? b->cap * 2 : 32; b->d = realloc(b->d, b->cap); }
    b->d[b->len++] = c;
}
static void bputs(Buf *b, const char *s) { while (*s) bputc(b, *s++); }
static char *bdetach(Buf *b) {
    if (!b->d) { b->d = malloc(1); }
    b->d[b->len] = 0;
    char *r = b->d;
    b->d = NULL; b->len = b->cap = 0;
    return r;
}

static char *xstrdup(const char *s) { size_t n = strlen(s); char *r = malloc(n + 1); memcpy(r, s, n + 1); return r; }
static char *fmtll(long long v) { char t[32]; sprintf(t, "%lld", v); return xstrdup(t); }
static char *pad6(long long v) {
    char t[32]; sprintf(t, "%lld", v);
    Buf b; binit(&b);
    for (size_t i = strlen(t); i < 6; i++) bputc(&b, '0');
    bputs(&b, t);
    return bdetach(&b);
}
static char *concat2(const char *a, const char *b) { Buf x; binit(&x); bputs(&x, a); bputs(&x, b); return bdetach(&x); }

static char *fmt6(double v) {
    int neg = 0;
    if (v < 0.0) { neg = 1; v = -v; }
    long long scaled = (long long)(v * 1000000.0 + 0.5);
    long long whole = scaled / 1000000;
    long long frac = scaled % 1000000;
    Buf b; binit(&b);
    if (neg) bputc(&b, '-');
    char t[32]; sprintf(t, "%lld", whole); bputs(&b, t);
    bputc(&b, '.');
    char f[32]; sprintf(f, "%06lld", frac); bputs(&b, f);
    return bdetach(&b);
}

static char *esc(const char *s) {
    Buf b; binit(&b); bputc(&b, '"');
    for (const char *p = s; *p; p++) {
        if (*p == '"') bputs(&b, "\\\"");
        else if (*p == '\\') bputs(&b, "\\\\");
        else if (*p == '\n') bputs(&b, "\\n");
        else if (*p == '\t') bputs(&b, "\\t");
        else if (*p == '\r') bputs(&b, "\\r");
        else bputc(&b, *p);
    }
    bputc(&b, '"');
    return bdetach(&b);
}

typedef enum { T_NULL, T_BOOL, T_INT, T_FLOAT, T_STR, T_ARR, T_OBJ } VT;
typedef struct Val {
    VT t;
    int b;
    long long i;
    double f;
    char *s;
    struct Val **items; int nitems;
    char **keys; struct Val **vals; int npairs;
} Val;

static Val *vnull(void) { Val *v = calloc(1, sizeof(Val)); v->t = T_NULL; return v; }
static Val *vbool(int b) { Val *v = calloc(1, sizeof(Val)); v->t = T_BOOL; v->b = b; return v; }
static Val *vint(long long i) { Val *v = calloc(1, sizeof(Val)); v->t = T_INT; v->i = i; return v; }
static Val *vfloat(double f) { Val *v = calloc(1, sizeof(Val)); v->t = T_FLOAT; v->f = f; return v; }
static Val *vstr(char *s) { Val *v = calloc(1, sizeof(Val)); v->t = T_STR; v->s = s; return v; }
static Val *varr(void) { Val *v = calloc(1, sizeof(Val)); v->t = T_ARR; return v; }
static Val *vobj(void) { Val *v = calloc(1, sizeof(Val)); v->t = T_OBJ; return v; }
static void arr_add(Val *a, Val *x) { a->items = realloc(a->items, sizeof(Val *) * (a->nitems + 1)); a->items[a->nitems++] = x; }
static void obj_put(Val *o, char *k, Val *x) {
    o->keys = realloc(o->keys, sizeof(char *) * (o->npairs + 1));
    o->vals = realloc(o->vals, sizeof(Val *) * (o->npairs + 1));
    o->keys[o->npairs] = k; o->vals[o->npairs] = x; o->npairs++;
}

static int is_digit(int c) { return c >= '0' && c <= '9'; }
static int is_ws(int c) { return c == ' ' || c == '\t' || c == '\n' || c == '\r'; }
static int hex_val(int c) {
    if (c >= '0' && c <= '9') return c - '0';
    if (c >= 'a' && c <= 'f') return c - 'a' + 10;
    if (c >= 'A' && c <= 'F') return c - 'A' + 10;
    return -1;
}

typedef struct { const char *s; int n; int i; int err; } P;
static int p_peek(P *p) { return p->i < p->n ? (unsigned char)p->s[p->i] : -1; }
static void p_fail(P *p) { p->err = 1; }
static void p_skip_ws(P *p) { while (p->i < p->n && is_ws((unsigned char)p->s[p->i])) p->i++; }
static int p_lit(P *p, const char *lit) {
    int L = (int)strlen(lit);
    if (p->i + L > p->n) return 0;
    if (memcmp(p->s + p->i, lit, L) != 0) return 0;
    p->i += L;
    return 1;
}
static Val *parse_value(P *p);
static Val *parse_array(P *p);
static Val *parse_object(P *p);

static char *parse_u(P *p) {
    if (p->i + 4 > p->n) { p_fail(p); return xstrdup(""); }
    int code = 0;
    for (int k = 0; k < 4; k++) {
        int h = hex_val((unsigned char)p->s[p->i + k]);
        if (h < 0) { p_fail(p); return xstrdup(""); }
        code = code * 16 + h;
    }
    p->i += 4;
    Buf b; binit(&b);
    if (code == 9) bputc(&b, '\t');
    else if (code == 10) bputc(&b, '\n');
    else if (code == 13) bputc(&b, '\r');
    else if (code >= 32 && code <= 126) bputc(&b, (char)code);
    else { p_fail(p); }
    return bdetach(&b);
}

static char *parse_string(P *p) {
    p->i++;
    Buf b; binit(&b);
    while (1) {
        if (p->i >= p->n) { p_fail(p); return bdetach(&b); }
        int c = (unsigned char)p->s[p->i];
        if (c == '"') { p->i++; return bdetach(&b); }
        if (c == '\\') {
            p->i++;
            if (p->i >= p->n) { p_fail(p); return bdetach(&b); }
            int e = (unsigned char)p->s[p->i];
            if (e == 'n') { bputc(&b, '\n'); p->i++; }
            else if (e == 't') { bputc(&b, '\t'); p->i++; }
            else if (e == 'r') { bputc(&b, '\r'); p->i++; }
            else if (e == 'b') { bputc(&b, '\\'); bputc(&b, 'b'); p->i++; }
            else if (e == 'f') { bputc(&b, '\\'); bputc(&b, 'f'); p->i++; }
            else if (e == '"') { bputc(&b, '"'); p->i++; }
            else if (e == '\\') { bputc(&b, '\\'); p->i++; }
            else if (e == '/') { bputc(&b, '/'); p->i++; }
            else if (e == 'u') { p->i++; char *u = parse_u(p); bputs(&b, u); free(u); }
            else { p_fail(p); return bdetach(&b); }
        } else { bputc(&b, (char)c); p->i++; }
    }
}

static Val *parse_number(P *p) {
    int neg = 0;
    if (p_peek(p) == '-') { neg = 1; p->i++; }
    long long mant = 0;
    int ndig = 0;
    if (p_peek(p) == '0') { p->i++; ndig = 1; }
    else { while (is_digit(p_peek(p))) { mant = mant * 10 + (p_peek(p) - '0'); p->i++; ndig++; } }
    if (ndig == 0) { p_fail(p); return vnull(); }
    int isfloat = 0;
    long long scale = 1;
    if (p_peek(p) == '.') {
        isfloat = 1; p->i++;
        if (!is_digit(p_peek(p))) p_fail(p);
        while (is_digit(p_peek(p))) { mant = mant * 10 + (p_peek(p) - '0'); p->i++; scale *= 10; }
    }
    long long exp = 0;
    if (p_peek(p) == 'e' || p_peek(p) == 'E') {
        isfloat = 1; p->i++;
        int esign = 1;
        if (p_peek(p) == '+') p->i++;
        else if (p_peek(p) == '-') { esign = -1; p->i++; }
        if (!is_digit(p_peek(p))) p_fail(p);
        while (is_digit(p_peek(p))) { exp = exp * 10 + (p_peek(p) - '0'); p->i++; }
        exp *= esign;
    }
    if (isfloat) {
        double v = (double)mant / (double)scale;
        for (long long k = 0; k < exp; k++) v *= 10.0;
        for (long long k = 0; k < -exp; k++) v /= 10.0;
        if (neg) v = -v;
        return vfloat(v);
    }
    if (neg) mant = -mant;
    return vint(mant);
}

static Val *parse_array(P *p) {
    Val *a = varr();
    p->i++;
    p_skip_ws(p);
    if (p_peek(p) == ']') { p->i++; return a; }
    while (1) {
        Val *v = parse_value(p);
        arr_add(a, v);
        if (p->err) return a;
        p_skip_ws(p);
        int c = p_peek(p);
        if (c == ',') { p->i++; continue; }
        if (c == ']') { p->i++; return a; }
        p_fail(p);
        return a;
    }
}

static Val *parse_object(P *p) {
    Val *o = vobj();
    p->i++;
    p_skip_ws(p);
    if (p_peek(p) == '}') { p->i++; return o; }
    while (1) {
        p_skip_ws(p);
        if (p_peek(p) != '"') { p_fail(p); return o; }
        char *k = parse_string(p);
        p_skip_ws(p);
        if (p_peek(p) != ':') { p_fail(p); return o; }
        p->i++;
        Val *v = parse_value(p);
        obj_put(o, k, v);
        if (p->err) return o;
        p_skip_ws(p);
        int c = p_peek(p);
        if (c == ',') { p->i++; continue; }
        if (c == '}') { p->i++; return o; }
        p_fail(p);
        return o;
    }
}

static Val *parse_value(P *p) {
    p_skip_ws(p);
    if (p->i >= p->n) { p_fail(p); return vnull(); }
    int c = (unsigned char)p->s[p->i];
    if (c == '{') return parse_object(p);
    if (c == '[') return parse_array(p);
    if (c == '"') return vstr(parse_string(p));
    if (c == 't') { if (p_lit(p, "true")) return vbool(1); p_fail(p); return vnull(); }
    if (c == 'f') { if (p_lit(p, "false")) return vbool(0); p_fail(p); return vnull(); }
    if (c == 'n') { if (p_lit(p, "null")) return vnull(); p_fail(p); return vnull(); }
    return parse_number(p);
}

static long long sum_ints(Val *x) {
    if (x->t == T_INT) return x->i;
    if (x->t == T_ARR) { long long s = 0; for (int i = 0; i < x->nitems; i++) s += sum_ints(x->items[i]); return s; }
    if (x->t == T_OBJ) { long long s = 0; for (int i = 0; i < x->npairs; i++) s += sum_ints(x->vals[i]); return s; }
    return 0;
}
static void dump(Val *x, Buf *b) {
    char t[64];
    switch (x->t) {
        case T_NULL: bputs(b, "null"); break;
        case T_BOOL: bputs(b, x->b ? "true" : "false"); break;
        case T_INT: sprintf(t, "%lld", x->i); bputs(b, t); break;
        case T_FLOAT: { char *f = fmt6(x->f); bputs(b, f); free(f); } break;
        case T_STR: { char *e = esc(x->s); bputs(b, e); free(e); } break;
        case T_ARR:
            bputc(b, '[');
            for (int i = 0; i < x->nitems; i++) { if (i) bputc(b, ','); dump(x->items[i], b); }
            bputc(b, ']');
            break;
        case T_OBJ:
            bputc(b, '{');
            for (int i = 0; i < x->npairs; i++) {
                if (i) bputc(b, ',');
                char *e = esc(x->keys[i]); bputs(b, e); free(e);
                bputc(b, ':');
                dump(x->vals[i], b);
            }
            bputc(b, '}');
            break;
    }
}

/* ---- deterministic generator (mirrors the sloth source) ---- */
static int64_t seed = 987654321;
static int64_t rnd(void) { seed = (seed * 1103515245 + 12345) % 2147483648LL; return seed; }
static char *gen_str_content(void) {
    int n = (int)(rnd() % 16);
    Buf b; binit(&b);
    for (int i = 0; i < n; i++) {
        int k = (int)(rnd() % 8);
        if (k == 0) bputc(&b, 'a');
        else if (k == 1) bputc(&b, 'Z');
        else if (k == 2) bputc(&b, '0');
        else if (k == 3) bputc(&b, ' ');
        else if (k == 4) bputc(&b, '"');
        else if (k == 5) bputc(&b, '\\');
        else if (k == 6) bputc(&b, '\n');
        else bputc(&b, '\t');
    }
    return bdetach(&b);
}
static char *gen_value(int depth) {
    int t = (int)(rnd() % 8);
    if (depth >= 3 && t >= 6) t %= 6;
    if (t == 0) return xstrdup("null");
    if (t == 1) return xstrdup("true");
    if (t == 2) return xstrdup("false");
    if (t == 3) { long long v = (long long)(rnd() % 200001) - 100000; return fmtll(v); }
    if (t == 4) {
        long long iv = (long long)(rnd() % 1000);
        long long fv = (long long)(rnd() % 1000000);
        char *p = fmtll(iv), *q = pad6(fv);
        char *dot = concat2(p, ".");
        char *r = concat2(dot, q);
        free(p); free(q); free(dot);
        return r;
    }
    if (t == 5) { char *c = gen_str_content(); char *e = esc(c); free(c); return e; }
    if (t == 6) {
        int n = (int)(rnd() % 6);
        Buf b; binit(&b); bputc(&b, '[');
        for (int i = 0; i < n; i++) { if (i) bputc(&b, ','); char *g = gen_value(depth + 1); bputs(&b, g); free(g); }
        bputc(&b, ']');
        return bdetach(&b);
    }
    int n = (int)(rnd() % 6);
    Buf b; binit(&b); bputc(&b, '{');
    for (int i = 0; i < n; i++) {
        if (i) bputc(&b, ',');
        char *c = gen_str_content(); char *e = esc(c); free(c);
        bputs(&b, e); free(e);
        bputc(&b, ':');
        char *g = gen_value(depth + 1); bputs(&b, g); free(g);
    }
    bputc(&b, '}');
    return bdetach(&b);
}

static void report(const char *src) {
    P p; p.s = src; p.n = (int)strlen(src); p.i = 0; p.err = 0;
    Val *v = parse_value(&p);
    p_skip_ws(&p);
    if (p.i != p.n) p_fail(&p);
    if (p.err) { printf("ERR\n"); return; }
    Buf b; binit(&b);
    dump(v, &b);
    char t[64]; sprintf(t, "|%lld", sum_ints(v));
    bputs(&b, t);
    char *out = bdetach(&b);
    printf("%s\n", out);
    free(out);
}

int main(void) {
    const char *cases[] = {
        "null", "true", "false", "42", "-7", "3.14", "1e3", "2.5e-2",
        "[]", "{}", "[1,2,3]", "  [ 1 , 2 ]  ",
        "{\"a\":1,\"b\":[true,null]}", "{\"x\":{\"y\":{\"z\":1}}}",
        "\"a\\\"b\"", "\"tab\\there\"", "\"\\u0041\\u0042\"", "\"slash\\/done\"",
        "{\"\":0}", "[[[[]]]]",
        "{", "[1,]", "tru", "\"abc", "01", "[1 2]", "nul", "-", "1.",
        "{\"a\" 1}", "\"\\x\"", "\"\\u00G1\"", "{\"a\":1,}",
        "0", "-0", "0.0", "0e0", "1E+2", "123.456e2",
        "\"\\u0020\\u007e\"", "\"\\u006a\\u004A\"", "\"caf\xc3\xa9\"", "\"\\u4e2d\"",
        "+1", "00", "-01", ".5", "1e", "1e+", "1.2.3", "[", "{\"a\":}", "{\"a\":1",
        "\r\n\t [\n1\r,\t2 ]\n",
        "{\"a\":1,\"a\":2}", "[[],{}]", "   ", "null,1"
    };
    int ncases = (int)(sizeof(cases) / sizeof(cases[0]));
    for (int i = 0; i < ncases; i++) report(cases[i]);
    {
        Buf b; binit(&b); bputc(&b, '[');
        for (int x = 0; x < 2000; x++) { if (x) bputc(&b, ','); char t[32]; sprintf(t, "%d", x); bputs(&b, t); }
        bputc(&b, ']');
        char *s = bdetach(&b); report(s); free(s);
    }
    {
        Buf b; binit(&b); bputc(&b, '{');
        for (int x = 0; x < 200; x++) {
            if (x) bputc(&b, ',');
            char t[32]; sprintf(t, "\"k%d\":%d", x, x); bputs(&b, t);
        }
        bputc(&b, '}');
        char *s = bdetach(&b); report(s); free(s);
    }
    {
        Buf b; binit(&b); bputc(&b, '"');
        for (int x = 0; x < 400; x++) bputc(&b, 'a');
        bputc(&b, '"');
        char *s = bdetach(&b); report(s); free(s);
    }
    {
        Buf b; binit(&b);
        for (int y = 0; y < 300; y++) bputc(&b, '[');
        for (int y = 0; y < 300; y++) bputc(&b, ']');
        char *s = bdetach(&b); report(s); free(s);
    }
    {
        Buf b; binit(&b); bputc(&b, '[');
        for (int y = 0; y < 1000; y++) { if (y) bputc(&b, ','); bputs(&b, "[]"); }
        bputc(&b, ']');
        char *s = bdetach(&b); report(s); free(s);
    }
    for (int t = 0; t < 600; t++) { char *src = gen_value(0); report(src); free(src); }
    return 0;
}
