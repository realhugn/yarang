.section .rodata

.global mutex_db_start;
.global mutex_db_end;
.global mutex_db_size;

mutex_db_start: 
mutex_db_end:
mutex_db_size: .long mutex_db_end - mutex_db_start
