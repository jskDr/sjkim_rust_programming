#include <iostream>
#include <stack>
#include <queue>
using namespace std;

class BTN {
public:
    int val;
    BTN* left;
    BTN* right;
    BTN(int v) : val(v), left(nullptr), right(nullptr) {}
};

void print_preorder(BTN *root) {
    stack<BTN*> s;
    BTN *cur = root;
    while (cur != nullptr || !s.empty()) {
        while (cur != nullptr) {
            cout << cur->val << " ";
            s.push(cur);
            cur = cur->left;
        }
        cur = s.top();
        s.pop();
        cur = cur->right;
    }
    cout << endl;
}

void print_preorder2(BTN *root) {
    stack<BTN*> s;
    s.push(root);
    while (!s.empty()) {
        BTN *node = s.top();
        s.pop();
        cout << node->val << " ";
        if (node->right) s.push(node->right);
        if (node->left) s.push(node->left);
    } 
    cout << endl;
}

void print_levelorder(BTN *root) {
    queue<BTN*> q;
    q.push(root);
    while (!q.empty()) {
        BTN *node = q.front();
        q.pop();
        cout << node->val << " ";
        if (node->right) q.push(node->left);
        if (node->left) q.push(node->right);
    } 
    cout << endl;
}

void print_inorder(BTN *root) {
    stack<BTN*> s;
    BTN *cur = root;
    while (cur != nullptr || !s.empty()) {
        while (cur != nullptr) {
            s.push(cur);
            cur = cur->left;
        }
        cur = s.top();
        cout << cur->val << " ";
        s.pop();
        cur = cur->right;
    }
    cout << endl;
}

void print_postorder(BTN *root) {
    stack<BTN*> s;
    BTN *cur = root;
    // 추가 stack을 이용해 cur가 right가 있었던 경우를 구분
    stack<BTN*> cur_stack;
    while (cur != nullptr || !s.empty()) {
        while (cur != nullptr) { 
            s.push(cur); // (1, 2, 4), (1, 2, 5)
            cur = cur->left;
        }
        cur = s.top(); // 4, 2, 5, 2
        if (cur->right) { // nullptr, 5, nullptr, 5
            if (cur_stack.empty() || cur != cur_stack.top()) {
                cur_stack.push(cur); // (2)
                cur = cur->right; // 5
                continue;
            }
            else {
                cur_stack.pop(); // ()
            }
        } 
        cout << cur->val << " "; // 4, 5, 2
        s.pop(); // (1, 2), (1, 2), (1)
        cur = nullptr; // , nullptr, 5
    }
    cout << endl;
}

void print_postorder1(BTN *root) { 
    //const BTN *root is not allowed because of BTN *cur = root. 
    // So, it is dangerous and *root value, original data, can be innocently changed.
    stack<BTN*> s;
    BTN *cur = root; // root should not be const
    while (cur != nullptr || !s.empty()) {
        while (cur != nullptr) {
            s.push(cur);
            cur = cur->left;
        }
        cur = s.top();
        if (cur->right) {
            BTN *tmp = cur;
            cur = cur->right;
            tmp->right = nullptr; // Warning: data is changed!
            continue;
        }
        cout << cur->val << " ";
        s.pop();
        cur = cur->right;
    }
    cout << endl;
}

void print_postorder2(BTN *root) {
    stack<BTN*> s;
    BTN *cur = root;
    // 추가 stack을 이용해 cur가 right가 있었던 경우를 구분
    stack<BTN*> cur_stack;
    while (cur != nullptr || !s.empty()) {
        while (cur != nullptr) { 
            s.push(cur); // (1, 2, 4), (1, 2, 5)
            cur = cur->left;
        }
        cur = s.top(); // 4, 2, 5, 2
        if (cur->right) { // nullptr, 5, nullptr, 5
            if (cur_stack.empty() || cur != cur_stack.top()) {
                cur_stack.push(cur); // (2)
                cur = cur->right; // 5
            }
            else {
                cur_stack.pop(); // ()
                cout << cur->val << " ";
                s.pop();
                cur = nullptr;
            }
        } else {
            cout << cur->val << " "; // 4, 5, 2
            s.pop(); // (1, 2), (1, 2), (1)
            cur = nullptr; // , nullptr, 5
        }
    }
    cout << endl;
}

void print_postorder3(BTN *root) {
    stack<BTN*> s;
    BTN *cur = root, *top = nullptr;
    while (cur != nullptr || !s.empty()) {
        while (cur != nullptr) { // 1, 2, 4, nullptr, 5, nullptr
            s.push(cur); // (1,2,4), (1,2,5)
            cur = cur->left; // 2,4,nullptr
        }
        cur = s.top(); // 4, 2, 5, 2
        if (cur->right && cur->right != top) { // nullptr, (5 && 5 != 4), (5 && 5 != 5)
            cur = cur->right; // 5
            continue;
        }
        cout << cur->val << " "; // 4, 5, 2
        s.pop(); // (1,2), (1,2), (1) 
        top = cur; // top: 4, 5, 2
        cur = nullptr; // nullptr
    }
    cout << endl;
}

int main() {
    BTN *root = new BTN(1);
    BTN *left = new BTN(2);
    BTN *right = new BTN(3);
    BTN *left_left = new BTN(4);
    BTN *left_right = new BTN(5);
    BTN *right_left = new BTN(6);
    BTN *right_right = new BTN(7);
    
    root->left = left;
    root->right = right;
    left->left = left_left;
    left->right = left_right;
    right->left = right_left;
    right->right = right_right;

    left->right->left = new BTN(8);
    left->right->right = new BTN(9);

    cout << "preorder: " << endl;
    print_preorder(root);
    print_preorder2(root);

    cout << "inorder: " << endl;
    print_inorder(root);    

    cout << "postorder: " << endl;
    print_postorder(root);
    print_postorder2(root);
    print_postorder3(root);

    cout << "levelorder: " << endl;
    print_levelorder(root);    
}