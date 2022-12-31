// File: findsetinset.cpp
// Created by James Sungjin Kim on 2022/12/29
// Description: Find a set in another set
// Including part
#include <iostream>
#include <iterator>
#include <vector>
#include <algorithm>
#include <fmt/ranges.h>
using namespace std;

// define xyz function with T type
template <typename T>
void print_vector(vector<T> &t) {
    copy(t.begin(), t.end(), ostream_iterator<T>(cout, " "));
    cout << endl;
}

bool binary_search(vector<int> &vec, int st, int ed, int target) {
    // print_vector<int>(vec);
    if(st <= ed) {
        int mid = (st + ed) / 2;
        if(vec[mid] == target) {
            return true;
        }
        else if(vec[mid] > target) {
            return binary_search(vec, st, mid-1, target);
        }
        else {
            return binary_search(vec, mid+1, ed, target);
        }
    }
    return false;
}

// Declartion part
bool findsetinset(const vector<int> &vec1, const vector<int> &vec2) {
    // To apply binary search, apply sorting first to vec1
    // Then, apply binary search to vec2
    // If all elements in vec2 are found in vec1, return true
    // Otherwise, return false
    vector<int> vec1_clone = vec1;
    sort(vec1_clone.begin(), vec1_clone.end(), less<int>());
    for(auto &i: vec2) {
        if(binary_search(vec1_clone, 0, vec1_clone.size()-1, i) == false) {
            return false;
        }
    }
    return true;
}

// Main part
void run(const vector<int> vec1, const vector<int> vec2) {
    // cout << vec1 << endl; // error because of no operator<< for vector
    // search method: search -> study for my problem -> apply (3step)
    // chatGPT method: chat -> apply (2step)
    cout << fmt::format("Given vector 1: {}\n", fmt::join(vec1, ", "));
    cout << fmt::format("Given vector 2: {}\n", fmt::join(vec2, ", "));
    bool result = findsetinset(vec1, vec2);
    cout << "After Processing, vector 1: ";
    copy(vec1.begin(), vec1.end(), ostream_iterator<int>(cout, " "));
    cout << " which should not be changed" << endl;
    cout << "Result: " << result << endl;
}

int main() {
    vector<int> v1_1 = {3, 7, 2, 9, 5};
    vector<int> v1_2 = {7, 9, 8};
    run(v1_1, v1_2);
    cout << "Check if result must be false." << endl;
    cout << endl;

    // same variable name can not be reused in C++
    vector<int> v2_1 = {3, 7, 2, 9, 5};
    vector<int> v2_2 = {7, 9, 5};
    run(v2_1, v2_2);
    cout << "Check if result must be true." << endl;

    return 0;
}