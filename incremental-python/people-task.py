import random

class People:
    def __init__(self, name,age,gender,alignment):
        self.name = name
        self.age = age
        self.gender = gender
        self.alignment = alignment
        self.damage = random.randint(2, 10)
        self.health = random.randint(50, 100)
        self.dead = False

    def __str__(self):
        return str(f"This person's name is {self.name}, they are {self.age} years old, they are a {self.gender}, and they are very {self.alignment}!!!")

    def hit(self, strength):
        self.health = self.health - strength
        if self.health <= 0:
            self.dead = True
            self.health = 0
            print(f'{self.name} is now Dead.')

    def attack(self):
        if not self.dead:
            return self.damage

def main():
    people = [
    ]

    blue = []
    for i in range(int(len(people)/2)):
        index = random.randint(0, len(people) - 1)
        blue.append(people[index])
        people.pop(index)

    red = people

    while len(red) >= 0 and len(blue) >= 0:
        if random.randint(1,2) == 1:
            attacking_team = blue
            target_team = red
        else:
            attacking_team = red
            target_team = blue
        
        attacker = attacking_team[random.randint(0, len(attacking_team) - 1)]
        target = target_team[random.randing(0, len(target_team) - 1)]
        target.hit(attacker.damage)
            

main()