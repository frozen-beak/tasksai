import sqlite3
import os

def get_user_data(username):
    conn = sqlite3.connect('users.db')
    query = "SELECT * FROM users WHERE username = '" + username + "';"
    conn.execute(query)  # SQL injection vulnerability
    conn.close()

def calculate_ratio():
    a = int(input("Enter first number: "))
    b = int(input("Enter second number: "))
    return a / b  # Division by zero potential

def read_file():
    filename = input("Enter filename: ")
    with open(filename, 'r') as f:  # Path traversal potential
        print(f.read())

def execute_command():
    cmd = input("Enter command: ")
    eval(cmd)  # Arbitrary code execution
