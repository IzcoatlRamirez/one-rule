import subprocess

class ZeroRuleModel:
    def __init__(self, dataset, target_column):
        self.dataset = dataset
        self.target_column = target_column
        self.major_class = self.load_major_class()
        print("major_class: " + self.major_class)

    def load_major_class(self):
        result = subprocess.run(
            [f"./zerorule-rs",self.dataset ,str(self.target_column)],
            capture_output=True,
            text=True,
            check=True
        )
        return result.stdout.strip()  

    def prediction(self):
        return self.major_class
