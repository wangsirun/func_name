// 一个良好的设计结构思想
struct string_list {
    // real stroge item container
	struct string_list_item *items;
    // record the nr in items
	size_t nr;
    // 记录了这个container的空间大小
    // 以备扩容之需
	size_t alloc;
    // controal field
	unsigned int strdup_strings:1;
    // some inner function with this struct
	compare_strings_fn cmp; /* NULL uses strcmp() */
};
---------------------------------------
static struct {
	const char *key;
	int enabled;
} advice_setting[] = {
	[ADVICE_ADD_EMBEDDED_REPO]			= { "addEmbeddedRepo", 1 },
	[ADVICE_ADD_EMPTY_PATHSPEC]			= { "addEmptyPathspec", 1 },
	[ADVICE_ADD_IGNORED_FILE]			= { "addIgnoredFile", 1 },
	[ADVICE_AM_WORK_DIR] 				= { "amWorkDir", 1 },
    }
---------------------------------------