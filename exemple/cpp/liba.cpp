// go wrapper c
#include <cstdint>
#include "liba.h"
#include <memory>
#include "vector"


#include "string"

// FABgen output .cpp
// This file is automatically generated, do not modify manually!



// const char * type tag
static uint32_t type_tag_const_char_ptr = 0x682eed33;


// char type tag
static uint32_t type_tag_char = 0x8cfe579f;


// unsigned char type tag
static uint32_t type_tag_unsigned_char = 0x838f8d38;


// uint8_t type tag
static uint32_t type_tag_uint8_t = 0x8eabc011;


// short type tag
static uint32_t type_tag_short = 0x8f2890a2;


// int16_t type tag
static uint32_t type_tag_int16_t = 0xbfd40c35;


// char16_t type tag
static uint32_t type_tag_char16_t = 0xc19fe7bb;


// uint16_t type tag
static uint32_t type_tag_uint16_t = 0xa32a93d6;


// unsigned short type tag
static uint32_t type_tag_unsigned_short = 0xc795d733;


// int32 type tag
static uint32_t type_tag_int32 = 0xe6bd962d;


// int type tag
static uint32_t type_tag_int = 0x1451dab1;


// int32_t type tag
static uint32_t type_tag_int32_t = 0x12d46c62;


// char32_t type tag
static uint32_t type_tag_char32_t = 0x6c9f87ec;


// size_t type tag
static uint32_t type_tag_size_t = 0x8883767d;


// uint32_t type tag
static uint32_t type_tag_uint32_t = 0xe2af381;


// unsigned int32_t type tag
static uint32_t type_tag_unsigned_int32_t = 0x92364bb7;


// unsigned int type tag
static uint32_t type_tag_unsigned_int = 0x657249f8;


// int64_t type tag
static uint32_t type_tag_int64_t = 0x2187e0e2;


// long type tag
static uint32_t type_tag_long = 0x3b97a968;


// float32 type tag
static uint32_t type_tag_float32 = 0xfd80ed46;


// float type tag
static uint32_t type_tag_float = 0xc9a55e95;


// intptr_t type tag
static uint32_t type_tag_intptr_t = 0xc10d8312;


// unsigned long type tag
static uint32_t type_tag_unsigned_long = 0x34e673cf;


// uint64_t type tag
static uint32_t type_tag_uint64_t = 0x3d797f01;


// double type tag
static uint32_t type_tag_double = 0xdae7f2ef;


// bool type tag
static uint32_t type_tag_bool = 0x55813692;


// std::string type tag
static uint32_t type_tag_string = 0x9ebeb2a9;


// basic interoperability
int return_int() { return 8; }
float return_float() { return 8.f; }
const char *return_const_char_ptr() { return "const char * -> string"; }

static int static_int = 9;

int *return_int_by_pointer() { return &static_int; }
int &return_int_by_reference() { return static_int; }

// argument passing
int add_int_by_value(int a, int b) { return a + b; }
int add_int_by_pointer(int *a, int *b) { return *a + *b; }
int add_int_by_reference(int &a, int &b) { return a + b; }

// type_tag based cast system
static bool _type_tag_can_cast(uint32_t in_type_tag, uint32_t out_type_tag) {
	if (out_type_tag == in_type_tag)
		return true;

	
	return false;
}

static void *_type_tag_cast(void *in_ptr, uint32_t in_type_tag, uint32_t out_type_tag) {
	if (out_type_tag == in_type_tag)
		return in_ptr;

	void *out_ptr = NULL;
	

	return out_ptr;
}

size_t gen_link_binding(gen_type_info *(*get_c_type_info)(const char *type)) {
	size_t unresolved = 0;

	return unresolved;
}

gen_type_info *gen_get_bound_type_info(uint32_t type_tag) {
	return nullptr;
}


gen_type_info *gen_get_c_type_info(const char *type) {
	return nullptr;
}


int  MyTestReturnInt(){
	auto ret = return_int();
return ret;
}
float  MyTestReturnFloat(){
	auto ret = return_float();
return ret;
}
const char * MyTestReturnConstCharPtr(){
	auto ret = return_const_char_ptr();
return (const char *)ret;
}
int * MyTestReturnIntByPointer(){
	auto ret = return_int_by_pointer();
return ret;
}
int * MyTestReturnIntByReference(){
	auto ret = &return_int_by_reference();
return ret;
}
int  MyTestAddIntByValue(int  a ,int  b){
	auto ret = add_int_by_value(a,b);
return ret;
}
int  MyTestAddIntByPointer(int * a ,int * b){
	auto ret = add_int_by_pointer(a,b);
return ret;
}
int  MyTestAddIntByReference(int * a ,int * b){
	auto ret = add_int_by_reference((int &)(*a),(int &)(*b));
return ret;
}
