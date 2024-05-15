#ifndef Value_h
#define Value_h

/* Value type */
enum class ValueType
{
    NUMBER,
};

/* Tagged union */
struct Value
{
    ValueType type;
    union
    {
        double number;
    };
};

/* Constructor */
#define NUMBER(value) Value{ValueType::NUMBER, {.number = value}}


// Accessor:
#define AS_NUMBER(value) ((double)(value).number)


#endif