#ifndef BOSONOGAIMPLEMENTATION_BOSONOGANAME_HPP
#define BOSONOGAIMPLEMENTATION_BOSONOGANAME_HPP

#include <string>
#include <functional>

struct BosonogaName {
  const std::string name;

  explicit BosonogaName(std::string name) : name(std::move(name)) {
  }

  [[nodiscard]] const std::string &str() const { return name; }

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
    return std::hash<std::string>{}(n.name);
  }
};

#endif // BOSONOGAIMPLEMENTATION_BOSONOGANAME_HPP
