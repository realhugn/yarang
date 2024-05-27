.section .rodata

.global regex_db_start;
.global regex_db_end;
.global regex_db_size;

regex_db_start: 
regex_db_end:
regex_db_size: .long regex_db_end - regex_db_start
