package main

import (
    "fmt"
    "time"
)

func main(){
    for {
        // fmt.Println(`{"name": "John Doe","age": 43,"phones": ["+44 1234567","+44 2345678"]}`);
        fmt.Println(`{"a": {"b": {"c": {"d": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9, 0]}}}}`)
        time.Sleep(1 * time.Second);
    }
}
