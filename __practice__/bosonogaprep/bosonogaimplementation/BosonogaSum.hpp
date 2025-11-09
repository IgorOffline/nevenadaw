#ifndef BOSONOGAIMPLEMENTATION_BOSONOGASUM_HPP
#define BOSONOGAIMPLEMENTATION_BOSONOGASUM_HPP

#include <functional>

struct BosonogaSum {
  const int sum;

  explicit BosonogaSum(const int v) : sum(v) {
  }

  [[nodiscard]] int to_int() const { return sum; }

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
    return std::hash<int>{}(s.sum);
  }
};

#endif // BOSONOGAIMPLEMENTATION_BOSONOGASUM_HPP
