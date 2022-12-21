trace_command_performance // 监测命令性能
skip_prefix // 去除前缀
handler_builtin // 一般是底层处理程序的入口
    {
        get_builtin 
        run_builtin
    }
die() // 一般是统一处理程序
setup_xxx // 一般是启动入口
setup_gently 
assert // yeah i get this
precompose_xxx // 聚合参数或者预处理参数
precious_objects // 宝贵的对象
partial_xxx // 处理的一部分
xxx_config // 
hash_algo // 算法可以使用词根algo
sparse_xxx //稀少的
unknown_extensions // 扩展
config_clear or config_reset
xxx_getcwd
die_error()
ancestor
fopen_or_warn

dry_run() // 排练
string_list_clear() // 重置
xxx_on_updating_xxx() // 更新操作
is_racy_timestamp() // 时间是否没有改变
smudge_racily_clean_xxx // 清除到原始状态
add_path_to_appropriate_result_list // 添加
sanitize // 清洁 净化
