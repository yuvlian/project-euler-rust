import requests
import os

base_dir = 'src/'

url_base = 'https://projecteuler.net/minimal='
start_problem = 6
end_problem = 100

for problem_number in range(start_problem, end_problem + 1):
    url = f"{url_base}{problem_number}"
    
    response = requests.get(url)
    response.raise_for_status()
    problem_description = response.text.strip()
    
    file_name = f"p_{problem_number}.rs"
    file_path = os.path.join(base_dir, file_name)
    
    rust_template = f"""/*
{problem_description}
*/

pub fn solution() {{
    // placeholder function
}}
"""

    with open(file_path, 'w') as file:
        file.write(rust_template)
    
    print(f"Created {file_path} for problem {problem_number}")

print("All files created successfully!")
