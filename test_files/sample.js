// Sample JavaScript file for testing CodeLens extension

function greet(name) {
  return `Hello, ${name}!`;
}

function calculateSum(a, b) {
  return a + b;
}

const PI = 3.14159;
let counter = 0;
var globalVar = "test";

class Calculator {
  constructor() {
    this.value = 0;
  }
  
  add(num) {
    this.value += num;
    return this;
  }
  
  multiply(num) {
    this.value *= num;
    return this;
  }
}

// Usage examples
console.log(greet("World"));
console.log(greet("Alice"));
console.log(greet("Bob"));

const sum = calculateSum(5, 3);
const anotherSum = calculateSum(PI, 2);

counter++;
counter += 5;
console.log(counter);

console.log(globalVar);
globalVar = "updated";

const calc = new Calculator();
calc.add(10).multiply(2);
console.log(calc.value);

const calc2 = new Calculator();
calc2.add(5);