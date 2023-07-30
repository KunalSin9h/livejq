package main

import (
    "fmt"
    "time"
)

func main(){
    for {
        // fmt.Println(`{"name": "John Doe","age": 43,"phones": ["+44 1234567","+44 2345678"]}`);
        fmt.Println(`{"a": {"b": {"c": {"d": [1, 2, 3]}}}}`)
        time.Sleep(1 * time.Second);
    }
}
