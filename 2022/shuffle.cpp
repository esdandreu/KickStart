#include <bits/stdc++.h>
#include <experimental/random>

int main(int argc, char** argv) {
    int n = 10;
    std::vector<int> deck;
    deck.reserve(n);
    for (int i = 1; i <= n; i++) {
        deck[i - 1] = i;
        std::cout << i << " ";
    }
    std::cout << std::endl;

    int choice;
    for (int i = 1; i <= n; i++) {
        choice = std::experimental::randint(i, n);
        std::iter_swap(deck.begin() + i - 1, deck.begin() + choice - 1);
        std::cout << deck[i - 1] << " ";
    }
    std::cout << std::endl;

    return 1;
}