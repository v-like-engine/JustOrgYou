import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import '../blocs/task_bloc.dart';
import '../widgets/task_list_item.dart';

class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('JustOrgYou'),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: () => context.read<TaskBloc>().add(LoadTasks()),
          ),
        ],
      ),
      body: BlocBuilder<TaskBloc, TaskState>(
        builder: (context, state) {
          if (state is TaskLoading) {
            return const Center(child: CircularProgressIndicator());
          }

          if (state is TaskError) {
            return Center(
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  const Icon(Icons.error_outline, size: 48),
                  const SizedBox(height: 16),
                  Text('Error: ${state.message}'),
                  const SizedBox(height: 16),
                  FilledButton(
                    onPressed: () => context.read<TaskBloc>().add(LoadTasks()),
                    child: const Text('Retry'),
                  ),
                ],
              ),
            );
          }

          if (state is TaskLoaded) {
            return SingleChildScrollView(
              padding: const EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  // Stats cards
                  _buildStatsCards(state),
                  const SizedBox(height: 24),

                  // Quick filters
                  const Text(
                    'Quick Filters',
                    style: TextStyle(
                      fontSize: 20,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  const SizedBox(height: 12),
                  _buildQuickFilters(context, state),
                  const SizedBox(height: 24),

                  // Recent tasks
                  const Text(
                    'Recent Tasks',
                    style: TextStyle(
                      fontSize: 20,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  const SizedBox(height: 12),
                  _buildRecentTasks(state),
                ],
              ),
            );
          }

          return const Center(child: Text('No tasks'));
        },
      ),
    );
  }

  Widget _buildStatsCards(TaskLoaded state) {
    return Row(
      children: [
        Expanded(
          child: _StatCard(
            title: 'TODO',
            count: state.todoTasks.length,
            color: Colors.blue,
            icon: Icons.check_box_outline_blank,
          ),
        ),
        const SizedBox(width: 12),
        Expanded(
          child: _StatCard(
            title: 'Inbox',
            count: state.inboxTasks.length,
            color: Colors.orange,
            icon: Icons.inbox,
          ),
        ),
        const SizedBox(width: 12),
        Expanded(
          child: _StatCard(
            title: 'Done',
            count: state.doneTasks.length,
            color: Colors.green,
            icon: Icons.check_box,
          ),
        ),
      ],
    );
  }

  Widget _buildQuickFilters(BuildContext context, TaskLoaded state) {
    return Wrap(
      spacing: 8,
      runSpacing: 8,
      children: [
        FilterChip(
          label: Text('Overdue (${state.overdueTasks.length})'),
          selected: false,
          onSelected: (selected) {
            // TODO: Filter by overdue
          },
          avatar: const Icon(Icons.warning, size: 16),
        ),
        FilterChip(
          label: Text('Waiting (${state.waitingTasks.length})'),
          selected: false,
          onSelected: (selected) {
            context.read<TaskBloc>().add(
                  const FilterTasks(keyword: 'WAITING'),
                );
          },
        ),
        FilterChip(
          label: Text('Someday (${state.somedayTasks.length})'),
          selected: false,
          onSelected: (selected) {
            context.read<TaskBloc>().add(
                  const FilterTasks(keyword: 'SOMEDAY'),
                );
          },
        ),
      ],
    );
  }

  Widget _buildRecentTasks(TaskLoaded state) {
    final recentTasks = state.tasks.take(10).toList();

    if (recentTasks.isEmpty) {
      return const Card(
        child: Padding(
          padding: EdgeInsets.all(24),
          child: Center(
            child: Text('No tasks yet. Add one to get started!'),
          ),
        ),
      );
    }

    return ListView.builder(
      shrinkWrap: true,
      physics: const NeverScrollableScrollPhysics(),
      itemCount: recentTasks.length,
      itemBuilder: (context, index) {
        return TaskListItem(task: recentTasks[index]);
      },
    );
  }
}

class _StatCard extends StatelessWidget {
  final String title;
  final int count;
  final Color color;
  final IconData icon;

  const _StatCard({
    required this.title,
    required this.count,
    required this.color,
    required this.icon,
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      elevation: 2,
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          children: [
            Icon(icon, color: color, size: 32),
            const SizedBox(height: 8),
            Text(
              count.toString(),
              style: TextStyle(
                fontSize: 24,
                fontWeight: FontWeight.bold,
                color: color,
              ),
            ),
            Text(
              title,
              style: const TextStyle(fontSize: 12),
            ),
          ],
        ),
      ),
    );
  }
}
