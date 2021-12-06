from pprint import pprint

# fn = "example.txt"
fn = "puzzle.txt"
data = [int(c) for c in open(fn).read().strip().split(",")]
age_max = max(max(data), 9)

population = {
    age: data.count(age)
    for age in range(age_max)
}
pprint(population)

for day in range(80):
    babies = population[0]
    for age in range(1, age_max):
        population[age-1] = population[age]
    population[8] = babies
    population[6] += babies
    pop_tot = sum(population.values())
    pprint((day, population, pop_tot))
