#include <iostream>
using namespace std;

template<class T>
class Node {
    public:
    T data;
    Node *next;
    Node(T data) {
        this->data = data;
        this->next = nullptr;
    }
};

// define template class
template<class T>
class SinglyLinkedList {
    public:
        Node<T> *p_root;
        SinglyLinkedList() {
            this->p_root = nullptr;
        }
        void append(T data) {
            Node<T> *p_new_node = new Node(data);
            if( !this->p_root) 
                this->p_root = p_new_node;
            else {
                Node<T> *p_root = this->p_root;
                while( p_root->next) {
                    p_root = p_root->next;
                }
                p_root->next = p_new_node;
            }
        }
        void remove_tail() {
            if(!this->p_root) {
                cout << "Error: No data to remove" << endl;
            } 
            else if(!this->p_root->next) {
                delete this->p_root;
                this->p_root = nullptr;
            } else {
                Node<T> *p_prev;
                Node<T> *p_root = this->p_root;
                while( p_root->next) {
                    p_prev = p_root;
                    p_root = p_root->next;
                }
                delete p_root;
                p_prev->next = nullptr;
            }
        }
        void print() {
            if(this->p_root) {
                Node<T> *p_root = this->p_root;
                while(p_root) {
                    cout << p_root->data;
                    if( p_root->next) cout << " -> ";
                    p_root = p_root->next;
                }
                cout << endl;
            }
            else {
                cout << "Error: Empty List Node!" << endl;
            }
        }
};


int main() {
    SinglyLinkedList<int> list = SinglyLinkedList<int>();
    list.print();
    
    list.append(10);
    list.print();

    list.append(20);
    list.append(30);
    list.print();
    
    list.remove_tail();
    list.print();

    list.remove_tail();
    list.remove_tail();
    list.print();
    return 0;
}