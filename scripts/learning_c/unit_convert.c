#include <ctype.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define ALIAS(x) x "\0"
#define UNIT(dimention, metric, name, to_base, alias)                                  \
  { dimention, metric, name, to_base, 0.0, alias "\0" }
#define UNIT_O(dimention, metric, name, to_base, offset, alias)                        \
  { dimention, metric, name, to_base, offset, alias "\0" }

#define METRIC(name, symbol, power)                                            \
  { name, symbol, power }

const char HELP[] = "Convert between units like this: \"12 mi => ft\".\n"
                    "Use \"quit\" to exit the program. Also note that\n"
                    "metric prefixes are supported on metric units.\n";

typedef enum { LENGTH, MASS, TEMPATURE } Dimention;

typedef struct {
  Dimention dimention;
  bool metric;
  char *name;
  float to_base;
  float offset;
  char *alias;
} Unit;

typedef struct {
  char *name;
  char *symbol;
  short power;
} MetricPrefix;

typedef struct {
  Unit unit;
  MetricPrefix prefix;
  bool found;
} FoundUnit;

typedef struct {
  char *name;
  MetricPrefix prefix;
} StrippedUnit;

const Unit UNITS[] = {
    UNIT(LENGTH, true,  "meter", 1.0,      ALIAS("m")),
    UNIT(LENGTH, false, "mile",  1609.344, ALIAS("mi")),
    UNIT(LENGTH, false, "inch",  0.0254,   ALIAS("in")),
    UNIT(LENGTH, false, "foot",  0.3048,   ALIAS("ft") ALIAS("feet")),
    UNIT(LENGTH, false, "yard",  0.9144,   ALIAS("yd")),

    UNIT(MASS, true,  "gram",  1.0,       ALIAS("g")),
    UNIT(MASS, false, "tonne", 1000000.0, ALIAS("t")),
    UNIT(MASS, false, "pound", 453.59237, ALIAS("lb") ALIAS("lbs")),

    UNIT_O(TEMPATURE, false, "celsius",    1.0,       0.0, ALIAS("c")),
    UNIT_O(TEMPATURE, false, "fahrenheit", 5.0 / 9.0, -32, ALIAS("f"))
};

const MetricPrefix PREFIXES[] = {
    METRIC("tera",  "T", 12),  METRIC("giga",  "G", 9),
    METRIC("mega",  "M", 6),   METRIC("kilo",  "k", 3),
    METRIC("hecto", "h", 2),   METRIC("deca",  "da", 1),
    METRIC("deci",  "d", -1),  METRIC("centi", "c", -2),
    METRIC("milli", "m", -3),  METRIC("micro", "μ", -6),
    METRIC("nano",  "n", -9),  METRIC("pico",  "p", -12),
};

const MetricPrefix DUMMY_PREFIX = METRIC("", "", 0);

const size_t UNIT_COUNT = sizeof(UNITS) / sizeof(Unit);
const size_t PREFIX_COUNT = sizeof(PREFIXES) / sizeof(MetricPrefix);

// Checks if string starts with prefix
bool starts_with(char *string, char *prefix) {
  return strncmp(prefix, string, strlen(prefix)) == 0;
}

// Remove any whitespace from the front of a string
void trim_front(char *str) {
  size_t i = 0;
  while (isspace(str[i]))
    i++;
  memmove(str, str + i, strlen(str) - i + 1);
}

// Moves the idx to the first non whitespace char
void skip_whitespace(char *str, size_t *idx) {
  while (isspace(str[*idx]))
    (*idx)++;
}

// Extract the next float in an input string, starting at idx
float read_float(char *str, size_t *idx) {
  while (!isdigit(str[*idx]) && str[*idx] != '\0')
    (*idx)++;
  char *start = &str[*idx];

  while (isdigit(str[*idx]) || str[*idx] == '.')
    (*idx)++;

  char *end = &str[*idx];
  return strtof(start, &end);
}

// Extract the next whitespace/'=' seperated substring, starting at idx
char *read_str(char *str, size_t *idx) {
  skip_whitespace(str, idx);
  char *start = &str[*idx];

  while (str[*idx] != '\0' && str[*idx] != '=' && !isspace(str[*idx]))
    (*idx)++;
  char *end = &str[*idx];

  char *out = malloc(end - start + 1);
  strncpy(out, start, end - start);
  out[end - start] = '\0';

  return out;
}

// Returns the index of the Unit in UNITS.
size_t find_unit(char *name) {
  for(size_t i = 0; name[i]; i++)
    name[i] = tolower(name[i]);

  for (size_t i = 0; i < UNIT_COUNT; i++) {
    Unit unit = UNITS[i];

    if (strcmp(unit.name, name) == 0)
      return i;

    for (size_t j = 0;;) {
      size_t length = strlen(unit.alias + j);
      if (length == 0)
        break;

      if (strcmp(unit.alias + j, name) == 0)
        return i;

      j += length + 1;
    }
  }

  return -1;
}

StrippedUnit strip_prefix(char *name) {
  StrippedUnit out;

  for (size_t i = 0; i < PREFIX_COUNT; i++) {
    MetricPrefix prefix = PREFIXES[i];

    if (starts_with(name, prefix.name)) {
      size_t len = strlen(prefix.name);
      out.name = name + len;
      out.prefix = prefix;
      return out;
    }

    if (starts_with(name, prefix.symbol)) {
      size_t len = strlen(prefix.symbol);
      out.name = name + len;
      out.prefix = prefix;
      return out;
    }
  }

  out.name = name;
  out.prefix = DUMMY_PREFIX;
  return out;
}

FoundUnit find_prefixed_unit(char *name) {
  FoundUnit out;

  size_t unit = find_unit(name);
  if (unit != -1) {
    out.found = true;
    out.unit = UNITS[unit];
    out.prefix = DUMMY_PREFIX;
    return out;
  }

  StrippedUnit strip = strip_prefix(name);
  unit = find_unit(strip.name);
  if (unit != -1 && UNITS[unit].metric) {
    out.found = true;
    out.unit = UNITS[unit];
    out.prefix = strip.prefix;
    return out;
  }

  return out;
}

char *dimention_name(Dimention dimention) {
  switch (dimention) {
  case LENGTH:
    return "length";
    break;
  case MASS:
    return "mass";
    break;
  case TEMPATURE:
    return "tempature";
    break;
  }
}

int main() {
  puts(HELP);

  size_t length = 35;
  fputs("The following units are supported: ", stdout);
  for (size_t i = 0; i < UNIT_COUNT; i++) {
    if (length > 40) {
      putchar('\n');
      length = 0;
    }

    Unit unit = UNITS[i];
    fputs(unit.name, stdout);
    length += strlen(unit.name) + 2;

    if (i + 1 != UNIT_COUNT)
      fputs(", ", stdout);
  }

  puts("\n");

  char *line = NULL;
  size_t size = 0;

  for (;;) {
    fputs("> ", stdout);
    if (getline(&line, &size, stdin) == -1)
      break;

    line[strcspn(line, "\n")] = '\0';
    trim_front(line);

    if (strcmp(line, "quit") == 0)
      break;

    size_t idx = 0;
    float base = read_float(line, &idx);
    char *start = read_str(line, &idx);

    skip_whitespace(line, &idx);
    if (strncmp(line + idx, "=>", 2)) {
      puts("Invalid conversion, no sepeator (=>)");
      free(start);
      continue;
    }

    idx += 2;

    char *end = read_str(line, &idx);

    FoundUnit start_unit = find_prefixed_unit(start);
    FoundUnit end_unit = find_prefixed_unit(end);

    if (!start_unit.found) {
      printf("Unknown start unit `%s`\n", start);
    } else if (!end_unit.found) {
      printf("Unknown end unit `%s`\n", end);
    } else {
      if (start_unit.unit.dimention != end_unit.unit.dimention) {
        char *start_dimention = dimention_name(start_unit.unit.dimention);
        char *end_dimention = dimention_name(end_unit.unit.dimention);
        printf("Dimention mismatch: %s vs %s\n", start_dimention,
               end_dimention);
      } else {
        short power = start_unit.prefix.power - end_unit.prefix.power;
        float power_mul = pow(10, power);
        
        float out = (base + start_unit.unit.offset) * start_unit.unit.to_base /
                        end_unit.unit.to_base - end_unit.unit.offset;
        out *= power_mul;
        
        printf(" \\ %f %s%s => %f %s%s\n", base, start_unit.prefix.name,
               start_unit.unit.name, out, end_unit.prefix.name, end_unit.unit.name);
      }
    }

    free(start);
    free(end);
  }

  free(line);
  return 0;
}

