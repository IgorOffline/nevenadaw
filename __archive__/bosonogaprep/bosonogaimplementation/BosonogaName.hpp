#ifndef BOSONOGAIMPLEMENTATION_BOSONOGANAME_HPP
#define BOSONOGAIMPLEMENTATION_BOSONOGANAME_HPP

#include <string>
#include <functional>
#include "BosonogaPrimitives.hpp"

struct BosonogaName {
  const bosonoga_string name;

  explicit BosonogaName(bosonoga_string name) : name(std::move(name)) {
  }

  [[nodiscard]] const bosonoga_string &str() const { return name; }

  friend bool operator==(const BosonogaName &a, const BosonogaName &b) {
    return a.name == b.name;
  }

  friend bool operator!=(const BosonogaName &a, const BosonogaName &b) {
    return !(a == b);
  }

  friend std::ostream &operator<<(std::ostream &os, const BosonogaName &n) {
    os << "BosonogaName[name=" << n.name << "]";
    return os;
  }
};

template<>
struct std::hash<BosonogaName> {
  size_t operator()(const BosonogaName &n) const noexcept {
    return std::hash<bosonoga_string>{}(n.name);
  }
};

#endif // BOSONOGAIMPLEMENTATION_BOSONOGANAME_HPP
