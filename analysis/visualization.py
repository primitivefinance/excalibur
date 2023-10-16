import matplotlib.pyplot as plt
import seaborn as sns
import pandas as pd

sns.set_theme(style="whitegrid")
sns.set_palette("tab10")


class Visualizer:
    def __init__(self, nrows, ncols, figsize=(16, 8)):
        self.nrows = nrows
        self.ncols = ncols
        self.fig, self.axes = plt.subplots(nrows=nrows, ncols=ncols, figsize=figsize)
        
        # If only one subplot, make axes a 2D array for consistency
        if nrows == 1 and ncols == 1:
            self.axes = [[self.axes]]
        elif nrows == 1:
            self.axes = [self.axes]
        elif ncols == 1:
            self.axes = [[ax] for ax in self.axes]

    def customize_plot(self, ax, title, xlabel, ylabel):
        ax.set_title(title, fontsize=16)
        ax.set_xlabel(xlabel, fontsize=14)
        ax.set_ylabel(ylabel, fontsize=14)
        ax.legend(fontsize=12)
        ax.grid(True)

    def plot_statistical(self, row, col, x_data, y_data, label, color=None):
        ax = self.axes[row][col]
        
        # Check if y_data is a list of DataFrames
        if isinstance(y_data, list) and all(isinstance(df, pd.DataFrame) for df in y_data):
            concatenated = pd.concat(y_data)
            mean_data = concatenated.groupby(concatenated.index).mean()
            std_data = concatenated.groupby(concatenated.index).std()

            sns.lineplot(x=x_data, y=mean_data, label=label, ax=ax, linewidth=2, color=color)
            ax.fill_between(x_data, mean_data - std_data, mean_data + std_data, color=color, alpha=0.2)

        # If y_data is a single Series
        else:
            print("y_data is a list of DataFrames of length 1, consider using `plot` instead!")
            sns.lineplot(x=x_data, y=y_data, label=label, ax=ax, linewidth=2, color=color)
            
        # Set title based on label
        self.customize_plot(ax, label, "X-Axis", "Y-Axis")

    def plot(self, row, col, x_data, y_data, labels, color=None):
        ax = self.axes[row][col]
    
        for x, y, lbl in zip(x_data, y_data, labels):
            sns.lineplot(x=x, y=y, label=lbl, ax=ax, linewidth=2, color=color)
            
        # Set title based on label
        self.customize_plot(ax, labels, "X-Axis", "Y-Axis")


    def save(self, filename):
        plt.tight_layout()
        plt.savefig(filename)