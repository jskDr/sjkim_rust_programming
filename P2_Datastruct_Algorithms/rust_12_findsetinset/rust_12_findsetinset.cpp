// File: findsetinset.cpp
// Created by James Sungjin Kim on 2022/12/29
// Description: Find a set in another set
// Including part
#include <iostream>
#include <iterator>
#include <vector>
#include <algorithm>
#include <unordered_map>
using namespace std;

// define xyz function with T type
template <typename T>
void print_vector(const vector<T> &t) {
    cout << "[";
    copy(t.begin(), t.end()-1, ostream_iterator<T>(cout, ", "));
    cout << *(t.end() - 1) << "]" << endl;
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
bool findsetinset_bs(const vector<int> &vec1, const vector<int> &vec2) {
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

bool findsetinset_basic(const vector<int> &vec1, const vector<int> &vec2) {
    for(auto val: vec2) {
        int i;
        for(i=0; i <vec1.size(); i++) {
            if(val == vec1[i]) {
                break;
            } 
        }
        if (i==vec1.size()) return false;
    }
    return true;
}

bool findsetinset_hash(const vector<int> vec1, const vector<int> vec2) {
    unordered_map<int, int> v2_map;
    for(auto &i: vec2) {
        v2_map[i] = 1;
    }
    int sum = 0;
    for(auto &i: vec1) {
        sum += v2_map.count(i);
    }
    if(sum==vec2.size()) return true;
    else return false;
}


// Main part
void run(const vector<int> vec1, const vector<int> vec2) {
    print_vector<int>(vec1);
    print_vector<int>(vec2);

    bool result;
    result = findsetinset_bs(vec1, vec2);
    cout << "Result: " << result << endl;

    result = findsetinset_basic(vec1, vec2);
    cout << "Result: " << result << endl;    

    result = findsetinset_hash(vec1, vec2);
    cout << "Result: " << result << endl;        
}

int main() {
    vector<int> v1_1 = {3, 7, 2, 9, 5};
    vector<int> v1_2 = {7, 9, 8};
    run(v1_1, v1_2);
    cout << "Check if result must be false." << endl;
    cout << endl;

    vector<int> v2_1 = {3, 7, 2, 9, 5};
    vector<int> v2_2 = {7, 9, 5};
    run(v2_1, v2_2);
    cout << "Check if result must be true." << endl;

    return 0;
}