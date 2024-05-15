#include <iostream>

#include "src/vm/VM.h"
#include "src/Logger.h"
#include "src/vm/Value.h"

int main(int argc, char const *argv[])
{
    VM vm;
    auto result = vm.exec(R"(42)");

    log(AS_NUMBER(result));

    std::cout << "All done!\n";

    return 0;
}