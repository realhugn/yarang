.section .rodata

.global literal_db_start;
.global literal_db_end;
.global literal_db_size;

literal_db_start: 
literal_db_end:
literal_db_size: .long literal_db_end - literal_db_start
