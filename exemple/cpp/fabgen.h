// FABgen .h

enum OwnershipPolicy { NonOwning, Copy, Owning };

static bool _type_tag_can_cast(uint32_t in_type_tag, uint32_t out_type_tag);
static void *_type_tag_cast(void *in_T0, uint32_t in_type_tag, uint32_t out_type_tag);

struct gen_type_info {
	uint32_t type_tag;
	const char *c_type;
	const char *bound_name;

	bool (*check)(void* p);
	void (*to_c)(void *p, void *out);
	int (*from_c)(void *obj, OwnershipPolicy policy);
};

// return a type info from its type tag
gen_type_info *gen_get_bound_type_info(uint32_t type_tag);
// return a type info from its type name
gen_type_info *gen_get_c_type_info(const char *type);
// returns the typetag of a userdata object, nullptr if not a Fabgen object
uint32_t gen_get_wrapped_object_type_tag(void* p);
