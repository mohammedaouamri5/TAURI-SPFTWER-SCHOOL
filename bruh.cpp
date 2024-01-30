#include"iostream"
#include"math.h"


int main(int argc, char const *argv[])
{

    const long long int N = 11; 
    const int A = 2; 
     for (size_t i = 1; i <= N; i++)
        printf("%d^%d %% %d = %d\n" ,A,i, N ,  (int)sqrt(A , i)%N  ); 
    
    return 0;
}