// Sample TypeScript file for testing CodeLens extension

interface User {
  id: number;
  name: string;
  email: string;
}

type Status = 'active' | 'inactive' | 'pending';

class UserService {
  private users: User[] = [];
  
  addUser(user: User): void {
    this.users.push(user);
  }
  
  findUser(id: number): User | undefined {
    return this.users.find(user => user.id === id);
  }
  
  updateStatus(id: number, status: Status): boolean {
    const user = this.findUser(id);
    if (user) {
      // user.status = status; // This would require extending the User interface
      return true;
    }
    return false;
  }
}

function processUser(user: User): string {
  return `Processing ${user.name} (${user.email})`;
}

const adminStatus: Status = 'active';
const userService = new UserService();

// Usage examples
const user1: User = {
  id: 1,
  name: "John Doe",
  email: "john@example.com"
};

const user2: User = {
  id: 2,
  name: "Jane Smith",
  email: "jane@example.com"
};

userService.addUser(user1);
userService.addUser(user2);

console.log(processUser(user1));
console.log(processUser(user2));

const foundUser = userService.findUser(1);
if (foundUser) {
  console.log(`Found: ${foundUser.name}`);
}

userService.updateStatus(1, adminStatus);
userService.updateStatus(2, 'inactive');

const newStatus: Status = 'pending';
console.log(`Status: ${newStatus}`);