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

void print_listorder(BTN *root) {
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

void print_postorder1(BTN *root) {
    stack<BTN*> s;
    BTN *cur = root;
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

    print_preorder(root);
    print_preorder2(root);

    print_listorder(root);

    print_inorder(root);    

    print_postorder(root);
    print_postorder2(root);
}