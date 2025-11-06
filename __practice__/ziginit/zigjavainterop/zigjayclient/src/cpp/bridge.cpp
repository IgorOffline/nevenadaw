#include <iostream>
#include <iomanip>
#include "json.hpp"

extern "C" {

nlohmann::json* createJsonMaybe() {
    return new nlohmann::json{
        {"maybe", -1}
    };
}

void printJson(void* ptr) {
    auto* j = static_cast<nlohmann::json*>(ptr);
    std::cout << std::setw(4) << *j << std::endl;
}

void freeJson(void* ptr) {
    auto* j = static_cast<nlohmann::json*>(ptr);
    delete j;
}

}