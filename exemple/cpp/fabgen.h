// FABgen .h

enum OwnershipPolicy { NonOwning, Copy, Owning };


// basic interoperability
int return_int() ;
float return_float() ;
const char *return_const_char_ptr() ;


int *return_int_by_pointer() ;
int &return_int_by_reference() ;

// argument passing
int add_int_by_value(int a, int b) ;
int add_int_by_pointer(int *a, int *b) ;
int add_int_by_reference(int &a, int &b) ;

