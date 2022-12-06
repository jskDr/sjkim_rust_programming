// Binary Tree Node
#include <iostream>
using namespace std;

struct btn {
    int data;
    btn *left; // NULL is available but 0 and NULL are not distinguishable
    btn *right;
};

void preorder_traversal(btn *root) {
    if (root == NULL) {
        return;
    }
    cout << root->data << endl;
    preorder_traversal(root->left);
    preorder_traversal(root->right);
}

int main() {
    btn *root = new btn;
    root->data = 1;
    root->left = NULL;
    root->right = NULL;
    btn *left = new btn;
    left->data = 2;
    left->left = NULL;
    left->right = NULL;
    btn *right = new btn;
    right->data = 3;
    right->left = NULL;
    right->right = NULL;
    /*
         1
        / \
       2   3
    */
    root->left = left; // rust not works
    root->right = right;
    
    // print root -> left -> right order
    preorder_traversal(root);

    return 0;
}