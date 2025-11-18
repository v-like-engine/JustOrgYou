# Getting Started with JustOrgYou

Welcome to JustOrgYou! This guide will help you get up and running quickly.

## What is JustOrgYou?

JustOrgYou is a modern task management application based on the GTD (Getting Things Done) methodology. It helps you:

- **Capture** all your tasks and ideas quickly
- **Organize** them into actionable categories
- **Process** your inbox regularly
- **Review** your commitments
- **Execute** your tasks efficiently

## Quick Start (5 minutes)

### Option 1: Docker (Recommended)

The fastest way to try JustOrgYou:

```bash
# Clone the repository
git clone https://github.com/yourusername/JustOrgYou.git
cd JustOrgYou

# Start the backend services
docker-compose up -d

# Open your browser to test the API
curl http://localhost:8000/api/v1/health
```

### Option 2: Local Development

For development or if you prefer not using Docker:

#### Backend Setup

```bash
cd backend

# Create virtual environment
python -m venv venv
source venv/bin/activate  # Windows: venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt

# Run the server
uvicorn app.main:app --reload
```

Backend will be available at http://localhost:8000

#### Frontend Setup

```bash
cd frontend

# Get Flutter dependencies
flutter pub get

# Run on your device/emulator
flutter run
```

## Understanding the GTD Workflow

### 1. Capture Everything

Use the **Quick Add** button to rapidly capture tasks:

- Tap the "+" floating action button
- Enter task title
- Hit Enter or tap "Add"
- Task goes to INBOX

**Best Practice**: Don't think about categorization during capture. Just get it out of your head!

### 2. Process Your Inbox

Regularly (ideally daily) process your inbox:

1. Go to the **Inbox** tab
2. For each task, decide:
   - Is it actionable?
     - Yes → Move to TODO, set priority/deadline
     - No → Delete or move to SOMEDAY
   - Does it require others?
     - Move to WAITING
   - Can you do it in 2 minutes?
     - Just do it now and mark DONE

### 3. Organize Your Tasks

**Task Categories:**

- **TODO**: Actionable tasks for this week
- **WAITING**: Blocked on someone/something
- **SOMEDAY**: Ideas for the future
- **DONE**: Completed tasks

**Priorities:**

- **A**: High priority, urgent
- **B**: Medium priority
- **C**: Low priority, nice to have

**Tags:** Use tags to categorize by context:
- `#work`, `#home`, `#errands`
- `#quick`, `#deep-work`
- `#email`, `#phone-call`

### 4. Review Regularly

**Daily Review** (5 minutes):
- Process inbox
- Check today's tasks
- Review upcoming deadlines

**Weekly Review** (30 minutes):
- Empty inbox completely
- Review all TODO tasks
- Check WAITING items
- Browse SOMEDAY list
- Plan next week

### 5. Execute

Focus on your TODO list:
- Filter by context tags (`#work` when at office)
- Sort by priority
- Check deadlines
- Start with high-priority items

## Key Features

### Quick Add

The fastest way to capture tasks:

1. Tap floating "+" button anywhere
2. Type task title
3. Hit Enter
4. Continue capturing more tasks

### Task Details

Tap any task to see/edit:
- Title and description
- Priority (A, B, C)
- Category (TODO, WAITING, etc.)
- Tags
- Scheduled date
- Deadline
- Custom properties

### Search

- **Text Search**: Type keywords to find tasks
- **Filter by Category**: Show only TODO, WAITING, etc.
- **Filter by Tag**: Show tasks with specific tags
- **AI Semantic Search** (requires backend): Search by meaning, not just keywords

### AI Features

When connected to the backend server:

#### Semantic Search
Find tasks by meaning:
- "tasks about website" finds "fix homepage bug", "update landing page"
- Works even if exact words don't match

#### Tag Suggestions
Get intelligent tag recommendations:
- Based on task content
- Learns from your existing tags
- Helps maintain consistency

#### Auto-Categorization
Automatically suggests categories:
- Analyzes task content
- Suggests TODO, WAITING, SOMEDAY, etc.
- Saves time during inbox processing

#### Duplicate Detection
Finds similar or duplicate tasks:
- Prevents redundant work
- Helps merge related tasks
- Keeps your list clean

## Tips for Success

### 1. Make Capture Frictionless

- Keep the app accessible
- Use quick add liberally
- Don't worry about perfection
- Better captured badly than forgotten

### 2. Process Regularly

- Set a daily alarm for inbox processing
- Make it a habit
- Keep inbox at zero daily
- Don't let it pile up

### 3. Use Contexts (Tags)

Create tags for different contexts:
```
#work, #home, #errands, #calls, #email
#online, #offline, #high-energy, #low-energy
```

Filter by context when you're in that situation.

### 4. Set Realistic Deadlines

- Only set deadlines for real deadlines
- Use scheduled dates for "would like to do by"
- Don't over-commit
- Review and adjust as needed

### 5. Review Consistently

Weekly review is critical:
- Schedule it in your calendar
- Make it non-negotiable
- Use a checklist
- Celebrate progress

## Data Management

### Backup Your Data

Your tasks are stored in plain text Org Mode files:

**Location:**
- Mobile: App's document directory
- Desktop: `~/.justorgyou/` (or custom location)

**Backup:**
```bash
# Manual backup
cp ~/.justorgyou/tasks.org ~/backups/

# Or sync with cloud storage
# Dropbox, Google Drive, iCloud, Syncthing, etc.
```

### Import/Export

#### Import from Org Mode

```
Settings → Import from Org Mode → Select file
```

#### Export to Org Mode

```
Settings → Export to Org Mode → Choose location
```

Your data is always accessible as plain text!

### Sync Between Devices

**Current Options:**

1. **Manual Sync**: Copy .org files via cloud storage
2. **File Sync Tools**: Use Dropbox, Syncthing, etc.
3. **Version Control**: Commit .org files to git

**Coming Soon:**
- Built-in sync service
- Conflict resolution
- Real-time updates

## Keyboard Shortcuts (Desktop)

- `Ctrl+N`: Quick add task
- `Ctrl+F`: Search tasks
- `Ctrl+R`: Refresh
- `Space`: Toggle task done
- `Delete`: Delete task
- `Enter`: Edit task

## Troubleshooting

### App Won't Start

1. Check if backend is running: `curl http://localhost:8000/api/v1/health`
2. Check logs: `docker-compose logs backend`
3. Restart services: `docker-compose restart`

### AI Features Not Working

1. Verify backend is running
2. Check API connection in Settings
3. First use downloads ML models (may take time)
4. Check backend logs for errors

### Sync Issues

1. Check file permissions
2. Verify cloud storage is syncing
3. Look for `.org.conflict` files
4. Use built-in merge tool (if available)

### Data Loss Prevention

- Backup regularly
- Use version control
- Enable cloud sync
- Test restore process

## Getting Help

- **Documentation**: Check the `docs/` folder
- **GitHub Issues**: Report bugs or request features
- **Discussions**: Ask questions, share tips
- **Examples**: See `examples/` for sample workflows

## Next Steps

1. **Read the full documentation** in `docs/`
2. **Set up your first tasks** and try the workflow
3. **Customize** tags and categories for your needs
4. **Join the community** to share your experience

## Recommended Reading

- "Getting Things Done" by David Allen
- Org Mode documentation
- GTD forums and communities

Welcome aboard! Happy organizing! 🎉
