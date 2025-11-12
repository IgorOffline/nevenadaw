#include "student.h"

#include <stdlib.h>
#include <string.h>

struct student_t {
  char *name;
};

student_t *student_create(const char *name) {
  student_t *student = malloc(sizeof(student_t));
  if (student && name) {
    const size_t len = strlen(name) + 1;
    student->name = malloc(len);
    memcpy(student->name, name, len);
  }
  return student;
}

void student_destroy(student_t *student) {
  if (student) {
    free(student->name);
    free(student);
  }
}

const char *student_get_name(const student_t *student) {
  return student ? student->name : NULL;
}

student_t *student_with_name(const student_t *original, const char *new_name) {
  if (!original) return NULL;

  student_t *modified = student_create(new_name);

  return modified;
}
