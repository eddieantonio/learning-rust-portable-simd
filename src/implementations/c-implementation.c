#include <stdlib.h>
#include <stdint.h>

size_t problem_1_for(int16_t *array, size_t n) {
    int positives = 0;
    int negatives = 0;

    for (size_t i = 0; i < n; i++) {
        positives += array[i] > 0;
        negatives += array[i] < 0;
    }

    return positives > negatives ? positives : negatives;
}

size_t problem_1_binary(int16_t *array, size_t n) {
    size_t low = 0;
    size_t high = n;

    /* Bisect left for index of first non-negative number: */
    while (low < high) {
        size_t midpoint = low + (high - low) / 2;

        if (array[midpoint] < 0) {
            low = midpoint + 1;
        } else {
            high = midpoint;
        }
    }

    size_t negatives = low;

    /* Bisect right for index of first positive number: */
    /* Low is kept at the same place. */
    low = 0;
    high = n;
    while (low < high) {
        size_t midpoint = low + (high - low) / 2;

        if (0 < array[midpoint]) {
            high = midpoint;
        } else {
            low = midpoint + 1;
        }
    }

    size_t positives = n - high;
    return positives > negatives ? positives : negatives;
}
