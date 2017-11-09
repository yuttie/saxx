#include <cstdint>
#include "esa.hxx"


template <typename Char, typename Index>
int simple_esaxx(
        Char const* const t,
        Index*      const sa,
        Index*      const l,
        Index*      const r,
        Index*      const d,
        Index       const n,
        Index       const k,
        Index&            m)
{
    return esaxx(t, sa, l, r, d, n, k, m);
}


#define def_esaxx(CharBits, IndexBits) int esaxx_c##CharBits##i##IndexBits(uint##CharBits##_t const* const t, int##IndexBits##_t* const sa, int##IndexBits##_t* const l, int##IndexBits##_t* const r, int##IndexBits##_t* const d, int##IndexBits##_t const n, int##IndexBits##_t const k, int##IndexBits##_t* const m) { return simple_esaxx(t, sa, l, r, d, n, k, *m); }
extern "C" {
    def_esaxx(8, 8)
    def_esaxx(8, 16)
    def_esaxx(8, 32)
    def_esaxx(8, 64)
    def_esaxx(16, 8)
    def_esaxx(16, 16)
    def_esaxx(16, 32)
    def_esaxx(16, 64)
    def_esaxx(32, 8)
    def_esaxx(32, 16)
    def_esaxx(32, 32)
    def_esaxx(32, 64)
    def_esaxx(64, 8)
    def_esaxx(64, 16)
    def_esaxx(64, 32)
    def_esaxx(64, 64)
}
