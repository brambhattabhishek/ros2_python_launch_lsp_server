from launch import LaunchDescription
from launch.actions import IncludeLaunchDescription
from launch.launch_description_sources import PythonLaunchDescriptionSource
import os

def generate_launch_description():
    return LaunchDescription([
        # LSP Use Case: Jump to 'node_launch_file.py'
        IncludeLaunchDescription(
            PythonLaunchDescriptionSource(
                os.path.join(os.path.dirname(__file__), 'node_launch_file.py')
            )
        )
    ])
