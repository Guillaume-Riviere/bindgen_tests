// FABgen .h

enum OwnershipPolicy { NonOwning, Copy, Owning };


// basic interoperability
int return_int() ;
float return_float() ;
const char *return_const_char_ptr() ;


int *return_int_by_pointer() ;
int &return_int_by_reference() ;