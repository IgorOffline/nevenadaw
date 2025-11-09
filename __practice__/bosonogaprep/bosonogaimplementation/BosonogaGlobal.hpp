#ifndef BOSONOGAIMPLEMENTATION_BOSONOGAGLOBAL_HPP
#define BOSONOGAIMPLEMENTATION_BOSONOGAGLOBAL_HPP

#include <utility>
#include <functional>
#include "BosonogaName.hpp"
#include "BosonogaSum.hpp"

struct BosonogaGlobal {
  const BosonogaName name;
  const BosonogaSum sum;

  explicit BosonogaGlobal(BosonogaName name_, const BosonogaSum sum_)
    : name(std::move(name_)), sum(sum_) {
  }

  friend bool operator==(const BosonogaGlobal &a, const BosonogaGlobal &b) {
    return a.name == b.name && a.sum == b.sum;
  }

  friend bool operator!=(const BosonogaGlobal &a, const BosonogaGlobal &b) {
    return !(a == b);
  }

  friend std::ostream &operator<<(std::ostream &os, const BosonogaGlobal &g) {
    os << "BosonogaGlobal[name=" << g.name << ", sum=" << g.sum << "]";
    return os;
  }
};

template<>
struct std::hash<BosonogaGlobal> {
  size_t operator()(const BosonogaGlobal &g) const noexcept {
    const size_t h1 = std::hash<BosonogaName>{}(g.name);
    const size_t h2 = std::hash<BosonogaSum>{}(g.sum);
    return h1 ^ h2 + 0x9e3779b97f4a7c15ULL + (h1 << 6) + (h1 >> 2);
  }
};

#endif // BOSONOGAIMPLEMENTATION_BOSONOGAGLOBAL_HPP
