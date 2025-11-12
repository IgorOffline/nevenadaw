#ifndef CSDLRAIN_STUDENT_H
#define CSDLRAIN_STUDENT_H

typedef struct student_t student_t;

student_t *student_create(const char *name);

void student_destroy(student_t *student);

const char *student_get_name(const student_t *student);

#endif  // CSDLRAIN_STUDENT_H
