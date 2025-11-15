#ifndef HOPETERNAL_HOPETERNAL_PRIMITIVES_H
#define HOPETERNAL_HOPETERNAL_PRIMITIVES_H

using hopeternal_int = std::int32_t;
inline std::ostream& hopeternal_cout = std::cout;
using hopeternal_endl_manipulator = std::ostream& (*)(std::ostream&);
constexpr hopeternal_endl_manipulator hopeternal_endl = std::endl;

#endif  // HOPETERNAL_HOPETERNAL_PRIMITIVES_H
