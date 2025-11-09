#ifndef BOSONOGAIMPLEMENTATION_BOSONOGAGLOBAL_HPP
#define BOSONOGAIMPLEMENTATION_BOSONOGAGLOBAL_HPP

#include <utility>
#include <functional>
#include <unordered_map>
#include "BosonogaName.hpp"
#include "BosonogaSum.hpp"

struct BosonogaGlobal {
  const std::unordered_map<BosonogaName, BosonogaSum> nameSum;

  explicit BosonogaGlobal(std::unordered_map<BosonogaName, BosonogaSum> nameSum_)
    : nameSum(std::move(nameSum_)) {
  }

  friend bool operator==(const BosonogaGlobal &a, const BosonogaGlobal &b) {
    return a.nameSum == b.nameSum;
  }

  friend bool operator!=(const BosonogaGlobal &a, const BosonogaGlobal &b) {
    return !(a == b);
  }

  friend std::ostream &operator<<(std::ostream &os, const BosonogaGlobal &g) {
    os << "BosonogaGlobal[nameSum={";
    bool first = true;
    for (const auto &[left, right]: g.nameSum) {
      if (!first) { os << ", "; }
      first = false;
      os << left << "->" << right;
    }
    os << "}]";
    return os;
  }
};

template<>
struct std::hash<BosonogaGlobal> {
  size_t operator()(const BosonogaGlobal &g) const noexcept {
    size_t h = BOSONOGA_ZERO;
    for (const auto &[left, right]: g.nameSum) {
      const size_t hk = std::hash<BosonogaName>{}(left);
      const size_t hv = std::hash<BosonogaSum>{}(right);
      h ^= hk ^ hv + 0x9e3779b97f4a7c15ULL + (hk << 6) + (hk >> 2);
    }
    return h;
  }
};

#endif // BOSONOGAIMPLEMENTATION_BOSONOGAGLOBAL_HPP
