
#include <stdint.h>
#include <sys/types.h>
extern "C"
{
	struct usb_ifc_info
	{
		/* from device descriptor */
		unsigned short dev_vendor;
		unsigned short dev_product;
		unsigned char dev_class;
		unsigned char dev_subclass;
		unsigned char dev_protocol;
		unsigned char ifc_class;
		unsigned char ifc_subclass;
		unsigned char ifc_protocol;
		unsigned char has_bulk_in;
		unsigned char has_bulk_out;
		unsigned char writable;
	};
}
