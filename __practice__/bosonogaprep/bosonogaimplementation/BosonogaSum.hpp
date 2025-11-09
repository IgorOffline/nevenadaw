#ifndef BOSONOGAIMPLEMENTATION_BOSONOGASUM_HPP
#define BOSONOGAIMPLEMENTATION_BOSONOGASUM_HPP

#include <functional>
#include "BosonogaPrimitives.hpp"

struct BosonogaSum {
  const bosonoga_int sum;

  explicit BosonogaSum(const bosonoga_int v) : sum(v) {
  }

  [[nodiscard]] bosonoga_int to_int() const { return sum; }

  friend bool operator==(const BosonogaSum &a, const BosonogaSum &b) {
    return a.sum == b.sum;
  }

  friend bool operator!=(const BosonogaSum &a, const BosonogaSum &b) {
    return !(a == b);
  }

  friend std::ostream &operator<<(std::ostream &os, const BosonogaSum &s) {
    os << "BosonogaSum[sum=" << s.sum << "]";
    return os;
  }
};

template<>
struct std::hash<BosonogaSum> {
  size_t operator()(const BosonogaSum &s) const noexcept {
    return std::hash<bosonoga_int>{}(s.sum);
  }
};

#endif // BOSONOGAIMPLEMENTATION_BOSONOGASUM_HPP
