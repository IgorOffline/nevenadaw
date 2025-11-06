#include <iostream>
#include <cstdint>

// Include the DLL header
#include "mathlib.h"

int main() {
    int32_t a = 10;
    int32_t b = 20;
    
    // Use the DLL function
    int32_t result = sum(a, b);
    
    std::cout << "Sum of " << a << " + " << b << " = " << result << std::endl;
    
    // Test with negative numbers
    int32_t c = -5;
    int32_t d = 15;
    int32_t result2 = sum(c, d);
    
    std::cout << "Sum of " << c << " + " << d << " = " << result2 << std::endl;
    
    return 0;
}