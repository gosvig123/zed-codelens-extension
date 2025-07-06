# Sample Python file for testing CodeLens extension

def greet(name):
    return f"Hello, {name}!"

def calculate_sum(a, b):
    return a + b

class Calculator:
    def __init__(self):
        self.value = 0
    
    def add(self, num):
        self.value += num
        return self
    
    def multiply(self, num):
        self.value *= num
        return self

class DataProcessor:
    def __init__(self, data):
        self.data = data
    
    def process(self):
        return [item * 2 for item in self.data]

# Variable assignments
PI = 3.14159
counter = 0
global_var = "test"

# Usage examples
print(greet("World"))
print(greet("Alice"))
print(greet("Bob"))

sum_result = calculate_sum(5, 3)
another_sum = calculate_sum(PI, 2)

counter += 1
counter += 5
print(counter)

print(global_var)
global_var = "updated"

calc = Calculator()
calc.add(10).multiply(2)
print(calc.value)

calc2 = Calculator()
calc2.add(5)

processor = DataProcessor([1, 2, 3, 4, 5])
result = processor.process()
print(result)

processor2 = DataProcessor([10, 20, 30])
result2 = processor2.process()
print(result2)