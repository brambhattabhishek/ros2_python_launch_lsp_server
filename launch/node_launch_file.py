from launch import LaunchDescription
from launch_ros.actions import Node

def generate_launch_description():
    return LaunchDescription([
        # LSP Use Case: Auto-complete available ROS2 nodes
        Node(
            package='publisher',
            executable='publisher',
            name='rust_publisher',
            output='screen',
            env={
                'MESSAGE_PREFIX': 'Hi',
                'MESSAGE_SUFFIX': 'There',
                'RATE': '1',
                'REPEAT': '5',
                'DEBUG': 'true'
            }
        ),
        # LSP Use Case: Validate parameter names and types
        Node(
            package='subscriber',
            executable='subscriber',
            name='rust_subscriber',
            output='screen',
            env={
                'OUTPUT_FILE': 'output.txt',
                'DEBUG': 'true'
            }
        )
    ])
