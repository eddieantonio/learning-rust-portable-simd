#include <stdlib.h>
#include <stdint.h>

size_t problem_1(int16_t *array, size_t n) {
    int positives = 0;
    int negatives = 0;
    
    for (size_t i = 0; i < n; i++) {
        positives += array[i] > 0;
        negatives += array[i] < 0;
    }

    return positives > negatives ? positives : negatives; 
}
