#include <iostream>
using namespace std;

class Piece{
protected:
    string _name;
    string _side;

    Piece(string name, string side){
        _name = name;
        _side = side;
    }

};

int main(){
    cout << "Waffle";
}