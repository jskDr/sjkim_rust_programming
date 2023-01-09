#include <iostream>
using namespace std;

template<class T>
class NodeT {
    public:
    T data;
    NodeT *next;
    NodeT(T data);
};

template<class T>
NodeT<T>::NodeT(T data) {
    this->data = data;
    this->next = nullptr;
}

void run_node() {
    NodeT<int> node = NodeT<int>(10);
    cout << node.data << endl;
}






// define template class
template<class T>
class SinglyLinkedList {
    public:
        NodeT<T> *p_root;
        SinglyLinkedList() {
            this->p_root = nullptr;
        }
        void append(T data) {
            NodeT<T> *p_new_node = new NodeT(data);
            if( !this->p_root) 
                this->p_root = p_new_node;
            else {
                NodeT<T> *p_root = this->p_root;
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
                NodeT<T> *p_prev;
                NodeT<T> *p_root = this->p_root;
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
                NodeT<T> *p_root = this->p_root;
                while(p_root) {
                    cout << p_root->data;
                    if( p_root->next) cout << " -> ";
                    p_root = p_root->next;
                }
                cout << endl;
            }
            else {
                cout << "Error: Empty List NodeT!" << endl;
            }
        }
};


void run_singly_linked_list() {
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
}

int main() {
    run_node();
    run_singly_linked_list();
    return 0;
}