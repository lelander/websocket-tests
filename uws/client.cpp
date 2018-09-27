#include <iostream>
#include <uWS/uWS.h>
#include <chrono>

using namespace std;
using namespace std::chrono;

int main()
{
  uWS::Hub h;

  microseconds start;
  int count = 0;
  int duration = 0;

  h.onMessage([&start,&count,&duration](uWS::WebSocket<uWS::CLIENT> *ws, char *message, size_t length, uWS::OpCode opCode) {
                auto end = duration_cast<microseconds>(system_clock::now().time_since_epoch());
                duration += (end - start).count();

                if (count < 100000) {
                  start = duration_cast<microseconds>(system_clock::now().time_since_epoch());
                  ws->send("1538011457915");
                  count++;
                } else {
                  cout << "Round trip time: " << duration/count << "us" << endl;
                }
              });

  h.onConnection([&start,&count](uWS::WebSocket<uWS::CLIENT> *ws, uWS::HttpRequest request) {
                   cout << "Connected!" << endl;
                   start = duration_cast<microseconds>(system_clock::now().time_since_epoch());
                   ws->send("1538011457915");
                   count+=1;
                 });

  h.connect("ws://localhost:3000");
  h.run();
}
