#ifndef VM_h
#define VM_h

#include <string>
#include <vector>
#include <array>
#include <sstream>

#include "../Logger.h"
#include "../bytecode/OpCode.h"
#include "./Value.h"

#define READ_BYTE() *ip++

// Gets a constan from the pool
#define GET_CONST() constants[READ_BYTE()]

// Stack top (Stacj overflow after exceeding).
#define STACK_LIMIT 512

// Binary operation
#define BINARY_OP(op)                \
    do {                             \
        auto op2 = AS_NUMBER(pop()); \
        auto op1 = AS_NUMBER(pop()); \
        push(NUMBER(op1 op op2));    \
    } while (false)

class VM
{
public:
    VM() {}

    /* REFENCE */
    uint8_t *ip;
    /*BYTECODE*/
    std::vector<uint8_t> code;
    /* Constrant pool */
    std::vector<Value> constants;
    /* STAK POINTEr */
    Value * sp;
    /* Operants stack */
    std::array<Value, STACK_LIMIT> stack;
    
    /* Push value into the stack */
    void push(const Value &value) {
        if ((size_t)(sp - stack.begin()) >= STACK_LIMIT){
            DIE << "Stack overflow\n";
        }
        *sp = value;
        sp++;
    }

    Value pop() {
        if (sp == stack.begin()) {
            DIE << "Emprt stack\n";
        }
        --sp;
        return *sp;
    }

    /* Execute VM */
    Value exec(const std::string &program)
    {

        // 2. Compile program to bytecode
        // code = compiler -> compile(ast);
        constants.push_back(NUMBER(10));
        constants.push_back(NUMBER(3));
        constants.push_back(NUMBER(10));

        // (- (* 10 3) 10)
        // code = {0x01, 0, 0x01, 1, 0x04, 0x01, 2, 0x03, 0x00};
        code = {OP_CONST, 0, OP_CONST, 1, OP_MUL, OP_CONST, 2, OP_SUB, OP_HALT};

        ip = &code[0];

        sp = &stack[0];

        return eval();
    }

    /*
    * Main eval loop
    */
    Value eval()
    {
        for (;;)
        {
            auto opcode = READ_BYTE();
            // log(opcode);
            switch (opcode)
            {
            case OP_HALT:
                return pop();

                // ----------------
                // Constants:

            case OP_CONST:
                push(GET_CONST());
                break;

                // ----------------
                // Math ops:

            case OP_ADD:
            {
                BINARY_OP(+);
                break;
            }

            case OP_SUB:
            {
                BINARY_OP(-);
                break;
            }

            case OP_MUL:
            {
                BINARY_OP(*);
                break;
            }
            
            case OP_DIV:
            {
                BINARY_OP(/);
                break;
            }

            default:
                DIE << "Unknown opcode: " << std::hex << opcode;
            }
        }
    }
};

#endif